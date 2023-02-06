using System;
using System.Reflection;

namespace LooCast
{
    using Core;
    using System.IO;

    public class MainEntrypoint : IEntrypoint
    {
        private CoreManager coreManager;

        public void PreInitialize()
        {
            string coreManagerPath = Path.Combine(Directory.GetParent(Directory.GetCurrentDirectory()).FullName, "Core", "CoreManager.cs");
            if (File.Exists(coreManagerPath))
            {
                Assembly assembly = Assembly.LoadFrom(coreManagerPath);
                Type type = assembly.GetType("LooCast.Core.CoreManager");
                coreManager = (CoreManager)Activator.CreateInstance(type);
                coreManager.PreInitialize();
            }
        }

        public void Initialize()
        {
            coreManager.Initialize();
        }

        public void PostInitialize()
        {
            coreManager.PostInitialize();
        }
    }
}
