﻿using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IComponentRegistry<KeyType, ValueType> : IGameObjectRegistry<KeyType, ValueType> where KeyType : IComponentIdentifier where ValueType : IComponentIdentifiable
    {
        
    }
}
