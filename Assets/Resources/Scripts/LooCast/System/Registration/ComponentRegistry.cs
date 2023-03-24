using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class ComponentRegistry : Registry<IComponentIdentifier, IComponentIdentifiable>, IComponentRegistry<IComponentIdentifier, IComponentIdentifiable>
    {
        public ComponentRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
