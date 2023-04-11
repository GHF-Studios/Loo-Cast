using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Movement
{
    [PersistableDataType]
    public abstract class MovementPersistableDataType<T> : PersistableDataType<T> where T : MovementPersistableData
    {
        
    }
}