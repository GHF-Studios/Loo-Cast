using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class DataRegistry : Registry<IDataIdentifier, IDataIdentifiable>, IDataRegistry<IDataIdentifier, IDataIdentifiable>
    {
        public DataRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
