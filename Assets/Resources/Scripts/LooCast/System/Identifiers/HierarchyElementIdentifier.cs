using System;

namespace LooCast.System.Identifiers
{
    using global::LooCast.System.Identifiers;

    [Serializable]
    public class HierarchyElementIdentifier : SystemObjectIdentifier
    {
        public HierarchyElementIdentifier(TypeIdentifier hierarchyElementTypeIdentifier, Guid hierarchyElementInstanceGUID, string gusid = null) : base(hierarchyElementTypeIdentifier, hierarchyElementInstanceGUID, gusid)
        {
            
        }
    }
}
