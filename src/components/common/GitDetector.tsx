import React, { useState } from 'react'
import { useGit } from '@/hooks/useGit'
import { Button } from '@/components/ui/button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

interface GitDetectorProps {
  projectPath: string
}

export const GitDetector: React.FC<GitDetectorProps> = ({ projectPath }) => {
  const { gitStatus, isLoading, error, checkGitStatus, initGitRepo, getAllGitConfig, gitConfig } =
    useGit()
  const [isInitializing, setIsInitializing] = useState(false)

  const handleCheckStatus = async () => {
    await checkGitStatus(projectPath)
  }

  const handleInitRepo = async () => {
    setIsInitializing(true)
    try {
      const success = await initGitRepo(projectPath, 'main')
      if (success) {
        // 初始化成功后，获取配置
        await getAllGitConfig(projectPath)
        // 重新检查状态
        await checkGitStatus(projectPath)
      }
    } finally {
      setIsInitializing(false)
    }
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Git Repository Status</CardTitle>
        <CardDescription>Manage your Git repository configuration</CardDescription>
      </CardHeader>
      <CardContent>
        {error && (
          <div className="mb-4 p-3 bg-red-50 border border-red-200 rounded-md text-red-800">
            {error}
          </div>
        )}

        {!gitStatus ? (
          <div className="text-center py-8">
            <p className="text-gray-500 mb-4">Click to check Git repository status</p>
            <Button onClick={handleCheckStatus} disabled={isLoading}>
              {isLoading ? 'Checking...' : 'Check Status'}
            </Button>
          </div>
        ) : (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-sm font-medium">Repository Status:</span>
              {gitStatus.isGitRepo ? (
                <Badge variant="default" className="bg-green-500">
                  ✓ Git Repository
                </Badge>
              ) : (
                <Badge variant="secondary" className="bg-gray-500">
                  ✗ Not a Git Repository
                </Badge>
              )}
            </div>

            {gitStatus.isGitRepo && (
              <>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <p className="text-sm text-gray-500">Git Version</p>
                    <p className="text-sm font-medium">{gitStatus.gitVersion || 'N/A'}</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">Current Branch</p>
                    <p className="text-sm font-medium">{gitStatus.branch || 'N/A'}</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">Total Commits</p>
                    <p className="text-sm font-medium">{gitStatus.commitCount || '0'}</p>
                  </div>
                  <div>
                    <p className="text-sm text-gray-500">Working Directory</p>
                    <p className="text-sm font-medium">
                      {gitStatus.isDirty ? (
                        <span className="text-orange-600">Modified</span>
                      ) : (
                        <span className="text-green-600">Clean</span>
                      )}
                    </p>
                  </div>
                </div>

                {gitConfig && (
                  <div className="mt-4 p-3 bg-gray-50 rounded-md">
                    <p className="text-sm font-medium mb-2">Git Configuration</p>
                    <div className="grid grid-cols-2 gap-2 text-sm">
                      <div>
                        <span className="text-gray-500">User Name:</span>{' '}
                        <span className="font-medium">{gitConfig.userName || 'Not set'}</span>
                      </div>
                      <div>
                        <span className="text-gray-500">Email:</span>{' '}
                        <span className="font-medium">{gitConfig.userEmail || 'Not set'}</span>
                      </div>
                    </div>
                  </div>
                )}
              </>
            )}

            {!gitStatus.isGitRepo && (
              <div className="mt-4 p-4 bg-yellow-50 border border-yellow-200 rounded-md">
                <p className="text-sm text-yellow-800 mb-3">
                  This directory is not a Git repository. Initialize one to enable version control.
                </p>
                <Button onClick={handleInitRepo} disabled={isInitializing || isLoading}>
                  {isInitializing ? 'Initializing...' : 'Initialize Git Repository'}
                </Button>
              </div>
            )}

            <Button
              onClick={handleCheckStatus}
              disabled={isLoading}
              variant="outline"
              className="w-full mt-4"
            >
              {isLoading ? 'Checking...' : 'Refresh Status'}
            </Button>
          </div>
        )}
      </CardContent>
    </Card>
  )
}
