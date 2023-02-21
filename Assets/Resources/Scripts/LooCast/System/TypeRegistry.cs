using System;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public class TypeRegistry : Registry<ITypeIdentifier, ITypeIdentifiable>, ITypeRegistry<ITypeIdentifier, ITypeIdentifiable>
    {
        public TypeRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
