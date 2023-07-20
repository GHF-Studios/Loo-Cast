using System;

namespace LooCast.System
{
    using global::LooCast.System.ECS;
    using global::LooCast.System.Initialization;
    using global::LooCast.System.Termination;

    public interface IManager : IEntity, IChild<IManager>, IParent<IManager>, 
                                IEarlyPreInitializationPhase, IPreInitializationPhase, ILatePreInitializationPhase, 
                                IEarlyInitializationPhase, IInitializationPhase, ILateInitializationPhase, 
                                IEarlyPostInitializationPhase, IPostInitializationPhase, ILatePostInitializationPhase, 
                                IEarlyPreTerminationPhase, IPreTerminationPhase, ILatePreTerminationPhase, 
                                IEarlyTerminationPhase, ITerminationPhase, ILateTerminationPhase, 
                                IEarlyPostTerminationPhase, IPostTerminationPhase, ILatePostTerminationPhase
    {
        #region Properties
        string ManagerName { get; }
        #endregion
    }
}
