using UnityEngine;
using LooCast.System;
using System.Collections.Generic;

public class HierarchyView : MonoBehaviour
{
    #region Fields
    [SerializeField] private GameObject hierarchyViewFolderPrefab;
    [SerializeField] private GameObject folderContainer;

    private IFolder rootHierarchyFolder;
    private Dictionary<string, HierarchyViewFolder> hierarchyViewFolderChildren;
    #endregion

    #region Unity Callbacks
    private void Start()
    {
        rootHierarchyFolder = MainManager.Instance;
        hierarchyViewFolderChildren = new Dictionary<string, HierarchyViewFolder>();
    }
    #endregion

    #region Methods
    #endregion
}
