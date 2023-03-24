using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class ResourceObjectRegistry : Registry<IResourceObjectIdentifier, IResourceObjectIdentifiable>, IResourceObjectRegistry<IResourceObjectIdentifier, IResourceObjectIdentifiable>
    {
        public ResourceObjectRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
