using System;
using System.Collections.Generic;

namespace LooCast.System.Hierarchies
{
    using LooCast.System.Identifiers;
    using LooCast.System.Registries;

    public class RegistryHierarchy : Hierarchy<IRegistry>
    {
        public RegistryHierarchy(HierarchyIdentifier hierarchyIdentifier, FolderPath hierarchyFolderPath, IHierarchyElement<IRegistry> rootElement, IHierarchy hierarchyParent) : base(hierarchyIdentifier, hierarchyFolderPath, rootElement, hierarchyParent)
        {
        }
    }
}
