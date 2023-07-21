using System;

namespace LooCast.System
{
    using LooCast.System.ECS;
    using LooCast.System.Lifecycle.Setup;
    using LooCast.System.Lifecycle.Initialization;
    using LooCast.System.Lifecycle.Termination;

    public interface IManager : IEntity, IChild<IManager>, IParent<IManager>,
                                IPreSetupPhase, ISetupPhase, IPostSetupPhase,
                                IEarlyPreInitializationPhase, ILatePreInitializationPhase, 
                                IEarlyInitializationPhase, ILateInitializationPhase, 
                                IEarlyPostInitializationPhase, ILatePostInitializationPhase,
                                IEarlyPreTerminationPhase, ILatePreTerminationPhase,
                                IEarlyTerminationPhase, ILateTerminationPhase,
                                IEarlyPostTerminationPhase, ILatePostTerminationPhase
    {
        #region Properties
        string ManagerName { get; }
        #endregion
    }
}
