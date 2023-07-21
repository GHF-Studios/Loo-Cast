using System;

namespace LooCast.System.ECS
{
    using LooCast.System.Lifecycle.Setup;
    using LooCast.System.Lifecycle.Initialization;
    using LooCast.System.Lifecycle.Termination;

    public interface IComponent : IPreInitializationPhase, IInitializationPhase, IPostInitializationPhase, IPreTerminationPhase, ITerminationPhase, IPostTerminationPhase
    {
        #region Properties
        Guid ComponentID { get; }
        IEntity Entity { get; }
        #endregion

        #region Methods
        void OnCreate();
        void OnDestroy();

        void Create_INTERNAL(IEntity entity);
        #endregion
    }
}
