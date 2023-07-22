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
        #region Interfaces
        new public interface IData : IEntity.IData
        {
            string ManagerName { get; set; }
            IManager ManagerParent { get; set; }
        }
        #endregion

        #region Properties
        string ManagerName { get; }
        #endregion
    }
}
