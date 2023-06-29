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
        ManagerMonoBehaviour ManagerMonoBehaviour { get; }
        #endregion

        #region Methods

        #region Initialization Action Registration
        void RegisterEarlyPreInitializationAction(Action action);

        void RegisterPreInitializationAction(Action action);

        void RegisterLatePreInitializationAction(Action action);

        void RegisterEarlyInitializationAction(Action action);

        void RegisterInitializationAction(Action action);

        void RegisterLateInitializationAction(Action action);

        void RegisterEarlyPostInitializationAction(Action action);

        void RegisterPostInitializationAction(Action action);

        void RegisterLatePostInitializationAction(Action action);
        #endregion

        #region Termination Action Registration
        void RegisterEarlyPreTerminationAction(Action action);

        void RegisterPreTerminationAction(Action action);

        void RegisterLatePreTerminationAction(Action action);

        void RegisterEarlyTerminationAction(Action action);

        void RegisterTerminationAction(Action action);

        void RegisterLateTerminationAction(Action action);

        void RegisterEarlyPostTerminationAction(Action action);

        void RegisterPostTerminationAction(Action action);

        void RegisterLatePostTerminationAction(Action action);
        #endregion
        
        #endregion
    }
}
