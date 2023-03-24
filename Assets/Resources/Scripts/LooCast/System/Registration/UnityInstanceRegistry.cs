﻿using System;

namespace LooCast.System.Registration
{
    using LooCast.System.Identification;
    using LooCast.System.Types;

    public class UnityInstanceRegistry : Registry<IUnityInstanceIdentifier, IUnityInstanceIdentifiable>, IUnityInstanceRegistry<IUnityInstanceIdentifier, IUnityInstanceIdentifiable>
    {
        public UnityInstanceRegistry(IType keyType, IType valueType) : base(keyType, valueType)
        {
            
        }
    }
}
