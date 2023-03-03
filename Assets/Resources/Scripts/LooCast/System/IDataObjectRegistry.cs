﻿using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IDataObjectRegistry<KeyType, ValueType> : IDataRegistry<KeyType, ValueType> where KeyType : IDataObjectIdentifier where ValueType : IDataObjectIdentifiable
    {
        
    }
}
