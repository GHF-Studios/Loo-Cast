using System;
using UnityEngine;

namespace LooCast.UI
{
    using UI.Animation;
    using UI.Bar;
    using UI.Button;
    using UI.Canvas;
    using UI.Cursor;
    using UI.HUD;
    using UI.Inspector;
    using UI.Inventory;
    using UI.Level;
    using UI.Overlay;
    using UI.Panel;
    using UI.Reward;
    using UI.Screen;
    using UI.Slider;
    using UI.Tab;
    using UI.Task;
    using UI.Timer;
    using UI.Title;
    using UI.Tooltip;
    using UI.Value;

    public class UIManager : ModuleManager
    {
        #region Static Properties
        public static UIManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[UIManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    DontDestroyOnLoad(instanceObject);
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<UIManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static UIManager instance;
        #endregion

        #region Fields

        #endregion

        #region Methods
        #endregion

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("UI", rootNamespace);
            looCastType = new Type(typeof(UIManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type versionInfoType = new Type(typeof(VersionInfo), looCastNamespace);

            typeManager.RegisterType(versionInfoType);
            #endregion
        }

        protected override SubModuleManager[] GetSubModuleManagers()
        {
            return new SubModuleManager[]
            {
                UIAnimationManager.Instance,
                UIBarManager.Instance,
                UIButtonManager.Instance,
                UICanvasManager.Instance,
                UICursorManager.Instance,
                UIHUDManager.Instance,
                UIInspectorManager.Instance,
                UIInventoryManager.Instance,
                UILevelManager.Instance,
                UIOverlayManager.Instance,
                UIPanelManager.Instance,
                UIRewardManager.Instance,
                UIScreenManager.Instance,
                UISliderManager.Instance,
                UITabManager.Instance,
                UITaskManager.Instance,
                UITimerManager.Instance,
                UITitleManager.Instance,
                UITooltipManager.Instance,
                UIValueManager.Instance
            };
        }
        #endregion
    }
}