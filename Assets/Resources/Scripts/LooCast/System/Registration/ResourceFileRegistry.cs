using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class ResourceFileRegistry : Registry<IResourceFileIdentifier, IResourceFileIdentifiable>, IResourceFileRegistry<IResourceFileIdentifier, IResourceFileIdentifiable>
    {
        public ResourceFileRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
