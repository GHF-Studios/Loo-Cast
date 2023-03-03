﻿using System;

namespace LooCast.System
{
    using LooCast.System.Identification;
    
    public interface IDataFileRegistry<KeyType, ValueType> : IDataObjectRegistry<KeyType, ValueType> where KeyType : IDataFileIdentifier where ValueType : IDataFileIdentifiable
    {
        
    }
}
