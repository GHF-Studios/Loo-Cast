using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class CSharpInstanceRegistry : Registry<ICSharpInstanceIdentifier, ICSharpInstanceIdentifiable>, ICSharpInstanceRegistry<ICSharpInstanceIdentifier, ICSharpInstanceIdentifiable>
    {
        public CSharpInstanceRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
