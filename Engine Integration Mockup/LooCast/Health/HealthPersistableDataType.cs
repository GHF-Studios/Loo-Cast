using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Health
{
    [PersistableDataType]
    public abstract class HealthPersistableDataType<T> : PersistableDataType<T> where T : HealthPersistableData
    {
        
    }
}