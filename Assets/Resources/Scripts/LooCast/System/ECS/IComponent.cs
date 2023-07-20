using System;

namespace LooCast.System.ECS
{
    public interface IComponent
    {
        #region Properties
        Guid ComponentID { get; }
        IEntity Entity { get; }
        #endregion

        #region Methods
        void Initialize_INTERNAL(IEntity entity);
        void OnCreate();
        void OnDestroy();
        void Destroy_INTERNAL();
        #endregion
    }
}
