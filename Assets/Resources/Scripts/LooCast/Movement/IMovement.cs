using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Movement
{
    using LooCast.Variable;

    public interface IMovement
    {
        #region Properties
        FloatComputedVariable Speed { get; }
        Rigidbody2D Rigidbody { get; }
        Collider2D Collider { get; }
        #endregion

        #region Events
        UnityEvent OnMovementEnabled { get; }
        UnityEvent OnMovementDisabled { get; }
        #endregion

        #region Methods
        void Accelerate();
        #endregion
    }
}
