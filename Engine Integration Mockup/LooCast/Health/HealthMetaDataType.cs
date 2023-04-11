using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Health
{
    [MetaDataType]
    public abstract class HealthMetaDataType<T> : MetaDataType<T> where T : HealthMetaData
    {
        
    }
}