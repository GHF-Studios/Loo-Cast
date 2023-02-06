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

        #region Methods
        void AccelerateInDirection(Vector3 targetDirection);
        void LookInDirection(Vector3 targetDirection);

        void AccelerateToPosition(Vector3 targetPosition);
        void LookAtPosition(Vector3 targetPosition);
        #endregion
    }
}
