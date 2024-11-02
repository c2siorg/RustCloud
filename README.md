# RustCloud
RustCloud  is a rust library which hides the difference between different APIs provided by varied cloud providers (AWS, GCP, Azure etc.) and allows you to manage different cloud resources through a unified and easy to use API.
<!-- 
![GoCloud Logo](assets/logo.png)

[![Codacy Badge](https://api.codacy.com/project/badge/Grade/0fce581810a6420aaca4ba6757c54529)](https://www.codacy.com/app/cloudlibz/gocloud?utm_source=github.com&utm_medium=referral&utm_content=cloudlibz/gocloud&utm_campaign=Badge_Grade)
[![Build Status](https://travis-ci.org/cloudlibz/gocloud.svg?branch=master)](https://travis-ci.org/cloudlibz/gocloud)
[![Slack](https://img.shields.io/badge/chat-on%20gitter-ff006f.svg?style=flat-square)](https://gitter.im/cloudlibz/gocloud)
[![docs](https://camo.githubusercontent.com/df8e028288079a740c10e6cfaad2fa0e0c96014d/687474703a2f2f696d672e736869656c64732e696f2f62616467652f446f63732d6c61746573742d677265656e2e737667)](docs) -->







<img src="assets/Rustcloud.png" width="650" height="300">


## Service Types

**Compute** -- Allows you to manage cloud and virtual servers.

**Database** -- Allows you to manage Compute storage.

**Container** -- Allows users to install and deploy containers onto container based virtualization platforms.

**Load balancer** -- Allows you to manager Load Balancer service.

**DNS** -- Allows you to manage DNS service.

## Service Providers

### AWS

* EC2 Compute [Link to example](examples/aws/compute/ec2.md)
* EKS Compute [Link to example](examples/aws/compute/eks.md)
* EC2 Storage [Link to example](examples/storage/aws_storage/aws_storage.md)
* Amazon Elastic Container Service (Container) [Link to example](examples/aws/compute/ecs.md)
* Elastic Load Balancing [Link to example](examples/aws/network/loadbalancer.md)
* AWS Route53 (DNS) [Link to example](examples/aws/network/dns.md)
* AWS DynamoDB (Database) [Link to example](examples/aws/database/dynamodb.md)
* AWS CloudWatch (Monitoring) [Link to example](examples/aws/management/monitoring.md)
* AWS IAM  [Link to example](examples/aws/security/iam.md)
* AWS Keymanagement [Link to example](examples/aws/security/kms.md)

### Google

* Google Compute [Link to example](examples/gcp/compute/compute_engine.md)
* Google Compute Storage [Link to example](examples/gcp/storage/storage.md)
* Google Kubernetes Service [Link to example](examples/gcp/compute/kubenetes.md)
* Google Elastic Load Balancing [Link to example](examples/gcp/network/loadbalancer.md)
* Google DNS [Link to example](examples/gcp/network/dns.md)
* Google Bigtable [Link to example](examples/gcp/database/bigtable.md)
* Google Notifications [Link to example](examples/gcp/app_services/notifications.md)

Currently, implementations for other cloud providers are being worked on.