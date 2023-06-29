using System;

namespace LooCast.System
{
    using LooCast.System.Initialization;
    using LooCast.System.Termination;

    public interface IManager : IFolder, IChild<IManager>, IParent<IManager>, 
                                IEarlyPreInitializationPhase, IPreInitializationPhase, ILatePreInitializationPhase, 
                                IEarlyInitializationPhase, IInitializationPhase, ILateInitializationPhase, 
                                IEarlyPostInitializationPhase, IPostInitializationPhase, ILatePostInitializationPhase, 
                                IEarlyPreTerminationPhase, IPreTerminationPhase, ILatePreTerminationPhase, 
                                IEarlyTerminationPhase, ITerminationPhase, ILateTerminationPhase, 
                                IEarlyPostTerminationPhase, IPostTerminationPhase, ILatePostTerminationPhase
    {
        #region Properties
        string ManagerName { get; }
        ExtendedMonoBehaviour ManagerMonoBehaviour { get; }
        #endregion
    }
}
