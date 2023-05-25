using System;
using System.Collections.Generic;

namespace LooCast.System.Registries
{
    using LooCast.System.Hierarchies;
    using LooCast.System.Identifiers;

    public class HierarchyRegistry : Registry<IHierarchyIdentifier, IHierarchy>
    {
        public HierarchyRegistry() : base(MainManager.Instance.MainRegistry)
        {
        }
    }
}
