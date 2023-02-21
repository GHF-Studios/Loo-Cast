using System;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public class InstanceRegistry : Registry<IInstanceIdentifier, IInstanceIdentifiable>, IInstanceRegistry<IInstanceIdentifier, IInstanceIdentifiable>
    {
        public InstanceRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
