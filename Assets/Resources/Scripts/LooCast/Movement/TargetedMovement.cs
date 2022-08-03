using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using LooCast.Target;

namespace LooCast.Movement
{
    using Data;
    using Target;

    public abstract class TargetedMovement : Movement
    {
        protected Target target;

        public void Initialize(TargetedMovementData data)
        {
            base.Initialize(data);
        }

        public virtual Target GetTarget()
        {
            return target;
        }

        public virtual void SetTarget(Target target)
        {
            this.target = target;
        }
    } 
}
