using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class ObjectRegistry : Registry<IObjectIdentifier, IObjectIdentifiable>, IObjectRegistry<IObjectIdentifier, IObjectIdentifiable>
    {
        public ObjectRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
