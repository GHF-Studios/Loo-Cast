using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement
{
    using LooCast.Variable;

    public interface IMovement
    {
        FloatComputedVariable Speed { get; }
        Rigidbody2D Rigidbody { get; }
        Collider2D Collider { get; }

        UnityEvent OnMovementEnabled { get; }
        UnityEvent OnMovementDisabled { get; }

        void Accelerate();
    } 
}
