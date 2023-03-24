using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class InstanceRegistry : Registry<IInstanceIdentifier, IInstanceIdentifiable>, IInstanceRegistry<IInstanceIdentifier, IInstanceIdentifiable>
    {
        public InstanceRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
