﻿using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IDataRegistry<KeyType, ValueType> : IObjectRegistry<KeyType, ValueType> where KeyType : IDataIdentifier where ValueType : IDataIdentifiable
    {
        
    }
}
