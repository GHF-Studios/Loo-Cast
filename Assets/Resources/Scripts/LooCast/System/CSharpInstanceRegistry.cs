using System;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public class CSharpInstanceRegistry : Registry<ICSharpInstanceIdentifier, ICSharpInstanceIdentifiable>, ICSharpInstanceRegistry<ICSharpInstanceIdentifier, ICSharpInstanceIdentifiable>
    {
        public CSharpInstanceRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
