using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class DataFileRegistry : Registry<IDataFileIdentifier, IDataFileIdentifiable>, IDataFileRegistry<IDataFileIdentifier, IDataFileIdentifiable>
    {
        public DataFileRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
