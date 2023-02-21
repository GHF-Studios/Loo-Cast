using System;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public class NamespaceRegistry : Registry<INamespaceIdentifier, INamespaceIdentifiable>, INamespaceRegistry<INamespaceIdentifier, INamespaceIdentifiable>
    {
        public NamespaceRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
