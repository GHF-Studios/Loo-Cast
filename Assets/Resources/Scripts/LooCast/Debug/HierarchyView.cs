using UnityEngine;
using LooCast.System;
using System.Collections.Generic;

public class HierarchyView : MonoBehaviour
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewFolderPrefab;
    [SerializeField] private GameObject hierarchyViewFilePrefab;
    
    private Dictionary<string, HierarchyViewFolder> hierarchyViewFolderChildren;
    private Dictionary<string, HierarchyViewFile> hierarchyViewFileChildren;
    #endregion

    #region Unity Callbacks
    private void Start()
    {
        Initialize();
    }
    #endregion

    #region Methods
    public void Initialize()
    {
        hierarchyViewFolderChildren = new Dictionary<string, HierarchyViewFolder>();
        hierarchyViewFileChildren = new Dictionary<string, HierarchyViewFile>();
        
        foreach (IFolder folder in MainManager.Instance.FolderChildren)
        {
            HierarchyViewFolder hierarchyViewFolder = Instantiate(hierarchyViewFolderPrefab, transform).GetComponent<HierarchyViewFolder>();
            hierarchyViewFolder.gameObject.name = folder.FolderName;
            hierarchyViewFolder.Initialize(folder);
            hierarchyViewFolderChildren.Add(folder.FolderName, hierarchyViewFolder);
        }

        foreach (IFile file in MainManager.Instance.FileChildren)
        {
            HierarchyViewFile hierarchyViewFile = Instantiate(hierarchyViewFilePrefab, transform).GetComponent<HierarchyViewFile>();
            hierarchyViewFile.gameObject.name = file.FileName;
            hierarchyViewFile.Initialize(file);
            hierarchyViewFileChildren.Add(file.FileName, hierarchyViewFile);
        }
    }
    #endregion
}
