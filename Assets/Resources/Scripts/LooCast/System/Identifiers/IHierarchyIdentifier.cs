using System;
using System.Collections.Generic;

namespace LooCast.System.Identifiers
{
    public interface IHierarchyIdentifier : IObjectIdentifier
    {
        new public string GUSID => HierarchyTypeIdentifier.GUSID;
        ITypeIdentifier HierarchyTypeIdentifier { get; }
    }
}
