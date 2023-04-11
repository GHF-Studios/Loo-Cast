using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Movement
{
    [MetaDataType]
    public abstract class MovementMetaDataType<T> : MetaDataType<T> where T : MovementMetaData
    {
        
    }
}