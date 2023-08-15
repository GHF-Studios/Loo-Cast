using System;
using System.Collections.Generic;
using System.Reflection;
using UnityEngine;

namespace LooCast.System
{
    using LooCast.System.ECS;

    [DisallowMultipleComponent]
    public abstract class ManagerUnityComponent : UnityComponent
    {
        #region Fields
        private Manager manager;
        private Action callback_Awake;
        private Action callback_Start;
        private Action callback_Update;
        private Action callback_LateUpdate;
        private Action callback_FixedUpdate;
        private Action callback_OnGUI;
        private Action callback_OnEnable;
        private Action callback_OnDisable;
        private Action callback_OnDestroy;

        private bool isSetup = false;
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            gameObject.layer = 31;
            gameObject.tag = "INTERNAL";
            DontDestroyOnLoad(this);
        }

        private void Start()
        {
            if (!isSetup)
            {
                throw new InvalidOperationException("Setup has not been done in time! Make sure that you call Setup() on this ManagerUnityComponent immediately after creating it!");
            }
            
            callback_Start.Invoke();
        }

        private void Update()
        {
            callback_Update.Invoke();
        }

        private void LateUpdate()
        {
            callback_LateUpdate.Invoke();
        }

        private void FixedUpdate()
        {
            callback_FixedUpdate.Invoke();
        }
        
        public virtual void OnGUI()
        {
            callback_OnGUI.Invoke();
        }

        public virtual void OnEnable()
        {
            callback_OnEnable.Invoke();
        }

        public virtual void OnDisable()
        {
            callback_OnDisable.Invoke();
        }

        public virtual void OnDestroy()
        {
            callback_OnDestroy.Invoke();
        }
        #endregion

        #region Methods
        public void Setup(Manager manager)
        {
            if (manager == null)
            {
                throw new ArgumentNullException(nameof(manager));
            }
            if (this.manager is not null)
            {
                throw new InvalidOperationException($"Manager reference has already been initialized!");
            }

            this.manager = manager;
            Type managerType = typeof(Manager);
            gameObject.name = manager.ManagerName; 
            
            MethodInfo onUnityAwakeMethodInfo = managerType.GetMethod("OnUnityAwake", BindingFlags.Instance, null, Type.EmptyTypes, null);
            MethodInfo onUnityStartMethodInfo = managerType.GetMethod("OnUnityStart", BindingFlags.Instance, null, Type.EmptyTypes, null);
            MethodInfo onUnityUpdateMethodInfo = managerType.GetMethod("OnUnityUpdate", BindingFlags.Instance, null, Type.EmptyTypes, null);
            MethodInfo onUnityLateUpdateMethodInfo = managerType.GetMethod("OnUnityLateUpdate", BindingFlags.Instance, null, Type.EmptyTypes, null);
            MethodInfo onUnityFixedUpdateMethodInfo = managerType.GetMethod("OnUnityFixedUpdate", BindingFlags.Instance, null, Type.EmptyTypes, null);
            MethodInfo onUnityGUIMethodInfo = managerType.GetMethod("OnUnityOnGUI", BindingFlags.Instance, null, Type.EmptyTypes, null);
            MethodInfo onUnityEnableMethodInfo = managerType.GetMethod("OnUnityOnEnable", BindingFlags.Instance, null, Type.EmptyTypes, null);
            MethodInfo onUnityDisableMethodInfo = managerType.GetMethod("OnUnityOnDisable", BindingFlags.Instance, null, Type.EmptyTypes, null);
            MethodInfo onUnityDestroyMethodInfo = managerType.GetMethod("OnUnityOnDestroy", BindingFlags.Instance, null, Type.EmptyTypes, null);

            if (onUnityAwakeMethodInfo != null)
            {
                callback_Awake = (Action)Delegate.CreateDelegate(typeof(Action), manager, onUnityAwakeMethodInfo);
            }
            if (onUnityStartMethodInfo != null)
            {
                callback_Start = (Action)Delegate.CreateDelegate(typeof(Action), manager, onUnityStartMethodInfo);
            }
            if (onUnityUpdateMethodInfo != null)
            {
                callback_Update = (Action)Delegate.CreateDelegate(typeof(Action), manager, onUnityUpdateMethodInfo);
            }
            if (onUnityLateUpdateMethodInfo != null)
            {
                callback_LateUpdate = (Action)Delegate.CreateDelegate(typeof(Action), manager, onUnityLateUpdateMethodInfo);
            }
            if (onUnityFixedUpdateMethodInfo != null)
            {
                callback_FixedUpdate = (Action)Delegate.CreateDelegate(typeof(Action), manager, onUnityFixedUpdateMethodInfo);
            }
            if (onUnityGUIMethodInfo != null)
            {
                callback_OnGUI = (Action)Delegate.CreateDelegate(typeof(Action), manager, onUnityGUIMethodInfo);
            }
            if (onUnityEnableMethodInfo != null)
            {
                callback_OnEnable = (Action)Delegate.CreateDelegate(typeof(Action), manager, onUnityEnableMethodInfo);
            }
            if (onUnityDisableMethodInfo != null)
            {
                callback_OnDisable = (Action)Delegate.CreateDelegate(typeof(Action), manager, onUnityDisableMethodInfo);
            }
            if (onUnityDestroyMethodInfo != null)
            {
                callback_OnDestroy = (Action)Delegate.CreateDelegate(typeof(Action), manager, onUnityDestroyMethodInfo);
            }

            callback_Awake.Invoke();

            isSetup = true;
        }
        #endregion
    }
}
