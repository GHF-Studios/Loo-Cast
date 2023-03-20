using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class TypeRegistry : Registry<ITypeIdentifier, ITypeIdentifiable>, ITypeRegistry<ITypeIdentifier, ITypeIdentifiable>
    {
        public TypeRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
