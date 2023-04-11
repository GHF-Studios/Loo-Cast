using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Movement
{
    [LogicType]
    public abstract class MovementLogicType<T> : LogicType<T> where T : MovementLogic
    {
        
    }
}