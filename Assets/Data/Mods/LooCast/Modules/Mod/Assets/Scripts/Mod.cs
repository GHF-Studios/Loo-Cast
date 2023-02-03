using System;
using System.IO;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Mod
{
    using Core;
    
    [Serializable]
    public class Mod
    {
        #region Properties
        public string ID
        {
            get
            {
                return Name;
            }
        }
        public string FolderPath
        {
            get
            {
                return Path.Combine(MainManager.ModsFolderPath, Name);
            }
        }
        public string ModulesFolderPath
        {
            get
            {
                return Path.Combine(FolderPath, "Modules");
            }
        }

        public string Name => name;
        public string DisplayName => displayName;
        public string Description => description;
        public string GameVersion => gameVersion;
        public string Version => version;
        public string Author => author;
        public string[] Dependencies => dependencies;
        public string[] Conflicts => conflicts;
        public string[] ModuleNames => moduleNames;
        #endregion

        #region Fields
        [SerializeField] private string name;
        [SerializeField] private string displayName;
        [SerializeField] private string description;
        [SerializeField] private string gameVersion;
        [SerializeField] private string version;
        [SerializeField] private string author;
        [SerializeField] private string[] dependencies;
        [SerializeField] private string[] conflicts;
        [SerializeField] private string[] moduleNames;
        #endregion

        #region Constructors
        public Mod(string name, string displayName, string description, string gameVersion, string version, string author, string[] dependencies, string[] conflicts, string[] moduleNames)
        {
            this.name = name;
            this.displayName = displayName;
            this.description = description;
            this.gameVersion = gameVersion;
            this.version = version;
            this.author = author;
            this.dependencies = dependencies;
            this.conflicts = conflicts;
            this.moduleNames = moduleNames;
            
            Validate();
        }
        #endregion

        #region Methods
        public virtual void OnPreInitialize()
        {
            
        }

        public virtual void OnInitialize()
        {

        }

        public virtual void OnPostInitialize()
        {

        }

        internal void Validate()
        {
            #region Mod Info Validation
            if (Name == null || Name == "")
            {
                throw new Exception($"[ModManager] Mod has no name!");
            }
            if (DisplayName == null || DisplayName == "")
            {
                throw new Exception($"[ModManager] Mod '{this}' has no display name!");
            }
            if (Description == null || Description == "")
            {
                throw new Exception($"[ModManager] Mod '{this}' has no description!");
            }
            if (GameVersion == null || GameVersion == "")
            {
                throw new Exception($"[ModManager] Mod '{this}' has no game version!");
            }
            if (Version == null || Version == "")
            {
                throw new Exception($"[ModManager] Mod '{this}' has no version!");
            }
            if (Author == null || Author == "")
            {
                throw new Exception($"[ModManager] Mod '{this}' has no author!");
            }
            if (Dependencies == null)
            {
                dependencies = new string[0];
            }
            if (Conflicts == null)
            {
                conflicts = new string[0];
            }
            if (ModuleNames == null || ModuleNames.Length == 0)
            {
                throw new Exception($"[ModManager] Mod '{Name}' doesn't contain any modules!");
            }
            #endregion

            #region File Structure Validation
            DirectoryInfo modulesFolder = new DirectoryInfo(ModulesFolderPath);
            if (!modulesFolder.Exists)
            {
                modulesFolder.Create();
            }
            #endregion

            #region Version Validation
            if (GameVersion != Application.version)
            {
                throw new Exception($"[ModManager] Mod '{Name}' has invalid game version '{GameVersion}'!");
            }
            #endregion

            #region Conflict Validation
            foreach (string conflict in Conflicts)
            {
                if (ModManager.Instance.LoadedMods.ContainsKey(conflict))
                {
                    throw new Exception($"[ModManager] Mod '{Name}' conflicts with '{conflict}'!");
                }
            }
            #endregion

            #region Dependency Validation
            foreach (string dependency in Dependencies)
            {
                if (!ModManager.Instance.LoadedMods.ContainsKey(dependency))
                {
                    throw new Exception($"[ModManager] Mod '{Name}' is missing dependency '{dependency}'!");
                }
            }
            #endregion

            #region Cyclic Dependency Validation
            HashSet<string> visited = new HashSet<string>();
            Stack<string> stack = new Stack<string>();
            visited.Add(Name);
            stack.Push(Name);

            while (stack.Count > 0)
            {
                string currentMod = stack.Pop();
                foreach (string dependency in Dependencies)
                {
                    if (!visited.Contains(dependency))
                    {
                        visited.Add(dependency);
                        stack.Push(dependency);
                    }
                    else
                    {
                        throw new Exception($"[ModManager] Mod {this} has a circular dependency on {dependency}!");
                    }
                }
            }
            #endregion
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return ID;
        }
        #endregion
    }
}
