using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class ResourceRegistry : Registry<IResourceIdentifier, IResourceIdentifiable>, IResourceRegistry<IResourceIdentifier, IResourceIdentifiable>
    {
        public ResourceRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
