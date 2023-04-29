using System;

namespace LooCast.System
{
    using LooCast.System.Types;
    
    public interface IManager : IComponentType.IComponent
    {
        #region Properties
        public ManagerObject ManagerObject { get; }
#nullable enable
        public IManager? ParentManager { get; }
#nullable disable

        #endregion

        #region Methods

        #region Initialization Action Registration
        public void RegisterEarlyPreInitializationAction(Action action);

        public void RegisterPreInitializationAction(Action action);

        public void RegisterLatePreInitializationAction(Action action);

        public void RegisterEarlyInitializationAction(Action action);

        public void RegisterInitializationAction(Action action);

        public void RegisterLateInitializationAction(Action action);

        public void RegisterEarlyPostInitializationAction(Action action);

        public void RegisterPostInitializationAction(Action action);

        public void RegisterLatePostInitializationAction(Action action);
        #endregion

        #region Termination Action Registration
        public void RegisterEarlyPreTerminationAction(Action action);

        public void RegisterPreTerminationAction(Action action);

        public void RegisterLatePreTerminationAction(Action action);

        public void RegisterEarlyTerminationAction(Action action);

        public void RegisterTerminationAction(Action action);

        public void RegisterLateTerminationAction(Action action);

        public void RegisterEarlyPostTerminationAction(Action action);

        public void RegisterPostTerminationAction(Action action);

        public void RegisterLatePostTerminationAction(Action action);
        #endregion
        
        #endregion
    }
}
