using System;
using UnityEngine;
using LooCast.Core;

namespace LooCast.Health
{
    [LogicType]
    public abstract class HealthLogicType<T> : LogicType<T> where T : HealthLogic
    {
        
    }
}