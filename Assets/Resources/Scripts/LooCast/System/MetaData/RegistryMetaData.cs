using System;
using UnityEngine;

namespace LooCast.System.MetaData
{
    using global::LooCast.System;
    using global::LooCast.System.Identifiers;

    [Serializable]
    public class RegistryMetaData : SystemObjectMetaData
    {
        public RegistryMetaData(TypeIdentifier typeIdentifier, SystemObject parentSystemObject = null, global::System.Collections.Generic.List<IMetaData> dependencies = null) : base(typeIdentifier, parentSystemObject, dependencies)
        {
            
        }
    }
}
