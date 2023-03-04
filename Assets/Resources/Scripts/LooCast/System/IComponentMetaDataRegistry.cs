﻿using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IComponentMetaDataRegistry<KeyType, ValueType> : IGameObjectMetaDataRegistry<KeyType, ValueType> where KeyType : IComponentMetaDataIdentifier where ValueType : IComponentMetaDataIdentifiable
    {
        
    }
}
