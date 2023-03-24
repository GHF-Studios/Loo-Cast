using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class DataObjectRegistry : Registry<IDataObjectIdentifier, IDataObjectIdentifiable>, IDataObjectRegistry<IDataObjectIdentifier, IDataObjectIdentifiable>
    {
        public DataObjectRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
