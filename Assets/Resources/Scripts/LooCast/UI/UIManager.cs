using System;
using UnityEngine;

namespace LooCast.UI
{
    using LooCast.System;
    using LooCast.System.Management;
    using LooCast.UI.Animation;
    using LooCast.UI.Bar;
    using LooCast.UI.Button;
    using LooCast.UI.Canvas;
    using LooCast.UI.Cursor;
    using LooCast.UI.HUD;
    using LooCast.UI.Inspector;
    using LooCast.UI.Inventory;
    using LooCast.UI.Level;
    using LooCast.UI.Overlay;
    using LooCast.UI.Panel;
    using LooCast.UI.Reward;
    using LooCast.UI.Screen;
    using LooCast.UI.Slider;
    using LooCast.UI.Tab;
    using LooCast.UI.Task;
    using LooCast.UI.Timer;
    using LooCast.UI.Title;
    using LooCast.UI.Tooltip;
    using LooCast.UI.Value;

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