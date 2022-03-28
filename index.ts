import { EmptyStatement, ModuleItem } from "@swc/core";
import Visitor from "@swc/core/Visitor";

export interface Opts {
  /**
   * The regular expression to match against import paths
   */
  pattern: RegExp;
}

export function createEmptyStatement(): EmptyStatement {
  return {
    type: "EmptyStatement",
    span: {
      start: 0,
      end: 0,
      ctxt: 0,
    },
  };
}

export function visitModuleItems(
  n: ModuleItem[],
  pattern: RegExp
): ModuleItem[] {
  return n.map((item) => {
      console.log(item);
    if (item.type == "ImportDeclaration" && pattern.test(item.source.value)) {
      return createEmptyStatement();
    }

    // messages.yaml --> messages
    if (item.type == "ImportDeclaration" && /messages\.yaml/.test(item.source.value)) {
        item.source.value.replace(/\.yaml/, '');
    }
    return item;
  });
}

export default class PluginIgnoreImport extends Visitor {
  private opts: Opts;

  constructor(opts: Opts) {
    super();
    this.opts = opts;
  }

  visitModuleItems(n: ModuleItem[]): ModuleItem[] {
    return visitModuleItems(n, this.opts.pattern);
  }
}
