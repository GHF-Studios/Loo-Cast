using System;

namespace LooCast.System
{
    using LooCast.System.Identification;

    public class UnityInstanceRegistry : Registry<IUnityInstanceIdentifier, IUnityInstanceIdentifiable>, IUnityInstanceRegistry<IUnityInstanceIdentifier, IUnityInstanceIdentifiable>
    {
        public UnityInstanceRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
