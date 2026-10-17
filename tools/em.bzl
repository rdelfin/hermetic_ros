"""Rules for expanding empy (`.em`) templates."""

def _em_expand_impl(ctx):
    package_dir = "/".join([p for p in [ctx.label.workspace_root, ctx.label.package] if p]) or "."

    args = ctx.actions.args()
    args.add("-o", ctx.outputs.out)
    for name, value in ctx.attr.defines.items():
        value = ctx.expand_location(value.replace("$(PACKAGE_DIR)", package_dir), ctx.attr.data)

        # empy runs `name=value` as Python; Starlark repr() is a valid Python string literal.
        args.add("-D", "{}={}".format(name, repr(value)))
    args.add(ctx.file.src)

    ctx.actions.run(
        executable = ctx.executable._em,
        arguments = [args],
        inputs = [ctx.file.src] + ctx.files.data,
        outputs = [ctx.outputs.out],
        mnemonic = "EmExpand",
        progress_message = "Expanding %{input}",
    )

em_expand = rule(
    implementation = _em_expand_impl,
    doc = "Expands an empy template into a single output file.",
    attrs = {
        "src": attr.label(
            allow_single_file = [".em"],
            mandatory = True,
            doc = "The empy template.",
        ),
        "out": attr.output(
            mandatory = True,
            doc = "The expanded file.",
        ),
        "data": attr.label_list(
            allow_files = True,
            doc = "Files the template reads or imports at expansion time.",
        ),
        "defines": attr.string_dict(
            doc = "Python string globals set before expansion. Values support " +
                  "$(location) expansion of `data`, and $(PACKAGE_DIR) for this " +
                  "package's path relative to the execroot.",
        ),
        "_em": attr.label(
            default = Label("//tools:em"),
            executable = True,
            cfg = "exec",
        ),
    },
)
